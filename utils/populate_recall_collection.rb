# Author: Nicholas Blantz
# Edited On: February 7, 2020

require 'json'
require 'open-uri'

# ------------------------- Variables ------------------------- #
# ------------------------------------------------------------- #

mongo_import_exe = "D:/Development/Databases/mongodb-4.2.3/bin/mongoimport.exe"
database_name = "cpscraper"
collection_name = "recalls"

# ------------------------ Don't touch ------------------------ #
# ------------------------------------------------------------- #

# A custom DateTime formatter for importing into MongoDB
class DateTimeFormatter
  def initialize(date_string)
    @date_string = date_string
  end
  
  def to_json(*a)
	"{\"$date\":\"#{@date_string}Z\"}"
  end
end

output_file = ".recall_data.json"

# Get recall json data from the cpsc recall endpoint
recall_json = JSON.parse(open('https://www.saferproducts.gov/RestWebServices/Recall?format=Json').read)

# Print reformatted JSON to output_file
open(output_file, 'wb') do |file|
  recall_json.each do |recall, index|
    recall["RecallDate"] = DateTimeFormatter.new(recall["RecallDate"])
	recall["LastPublishDate"] = DateTimeFormatter.new(recall["LastPublishDate"])
    file << recall.to_json + "\n"
  end
end

# Import JSON into MongoDB
import_result = system "#{mongo_import_exe} --db #{database_name} --collection #{collection_name} --drop --type json --file #{output_file}"

# Clear workspace and output result
File.delete(output_file) if File.exist?(output_file)
puts import_result
