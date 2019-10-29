#!/usr/bin/env python
# -*- coding: utf-8 -*-

# Install pymongo library version 3.5.1: pip install pymongo=3.5.1
# Install Cursor : https://github.com/GijsTimmers/cursor
#   pip install --user cursor

#import traceback
from pymongo import MongoClient
from Crypto.Cipher import AES
from os.path import expanduser
from datetime import datetime
import sys, hashlib, base64, json, os, re, time, cursor

mongo_username = "data_api"
mongo_password = "data_api"
mongo_port = "27017"
mongo_host = "localhost"

ext_file = "/etc/passwd"

output_dump_mongo = "./dumps_mongo"

def init_blockchain_simple(client):

    global mongo_collection

    mongo_db = client.sensors
    mongo_collection = mongo_db.sensors_values

def md5(fname):

    hash_md5 = hashlib.md5()

    with open(fname, "rb") as f:
        for chunk in iter(lambda: f.read(4096), b""):
            hash_md5.update(chunk)

    return hash_md5.hexdigest()

def sha256(fname):

    hash_sha256 = hashlib.sha256()

    with open(fname, "rb") as f:
        for chunk in iter(lambda: f.read(4096), b""):
            hash_sha256.update(chunk)

    return hash_sha256.hexdigest()

def decode_now(enc_data):

    md5_secret = md5(ext_file)
    sha256_secret = sha256(ext_file)

    secret_key = md5_secret
    secret_iv = md5_secret[:8] + sha256_secret[:8]

    key = hashlib.sha256(secret_key.encode('utf-8')).hexdigest()[0:32]
    iv = hashlib.sha256(secret_iv.encode('utf-8')).hexdigest()[0:16]

    decoded = base64.b64decode(enc_data)

    cipher = AES.new(key, AES.MODE_CBC, iv)

    decrypted = cipher.decrypt(decoded)

    final_string = decrypted.decode("utf-8")
    final_string = final_string.replace('\x0f', '')
    final_string = final_string.replace('\x05', '')
    final_string = final_string.replace('\t', '')
    final_string = final_string.replace('\"\"', '\"')

    return final_string.decode("utf-8")

def save_to_file(filename, contents):

    file = open(filename, "w")

    file.write(contents)
    file.close()

def main():

   args = sys.argv

   num_params = len(args)

   if num_params == 1:

      print ""
      print "Options for bulk_blockchain.py:"
      print ""
      print "     -type: Mandatory option to choose between databases"
      print "         simple: Bulk to files the data from the standard sensors blockchain (output path ./dumps_mongo/simple_blockchain)"
      print ""
      print "     Optional params:"
      print ""
      print "         -start-date: Start date to search with format dd-mm-YYY HH:mm:ss"
      print "         -end-date: End date to search with format dd-MM-YYY HH:mm:ss"
      print ""
      print "         \033[91m**No date parameters mean to get the full data timeline\033[0m"
      print ""
      print "Example:"
      print "           python bulk_blockchain.py -type=simple"
      print ""
      print "           python bulk_blockchain.py -type=simple -start-date=\"29-07-2019\" -end-date=\"31-07-2019\""
      print ""

      quit()

   elif num_params == 2:

      option = args[1].lower()

      if '-type=simple' in option:
          pass
      else:
          quit()

   elif num_params == 4:

      option = args[1].lower()
      start_str = args[2].lower()
      end_str = args[3].lower()

      if '-type=simple' in option and '-start-date=' in start_str and '-end-date=' in end_str:

          try:

            start_str = start_str.replace("-start-date=","")
            end_str = end_str.replace("-end-date=","")

            start_datetime = datetime.strptime(start_str, "%d-%m-%Y %H:%M:%S")
            end_datetime = datetime.strptime(end_str, "%d-%m-%Y %H:%M:%S")

            if (start_datetime < end_datetime):
                pass
            else:
                quit()

          except:
            quit()
      else:
          quit()
   else:
       quit()

   client_mongo = MongoClient('mongodb://' + mongo_username + ':' + mongo_password + '@' + mongo_host +':' + mongo_port)

   init_blockchain_simple(client_mongo)
   print "Connected to Simple blockchain"

   cursor.hide()

   if num_params == 2:
       cursor_mongo = mongo_collection.find()

   if num_params == 4:
       cursor_mongo = mongo_collection.find({"timestamp" : { "$gte" : start_datetime,  "$lte" : end_datetime }})

   num_docs = cursor_mongo.count()
   print "Number of documents: " + str(num_docs + 1)

   if num_docs > 0:

      print "Processing documents...."
      #print "Selected option: " + option.replace("-type=","")
      ouput_path = output_dump_mongo + "/" + ('simple_blockchain' if 'simple' in option else '')

      if not os.path.exists(ouput_path):
          os.makedirs(ouput_path)

      try:

          bulk_log_file = "bulk_blockchain.log"

          with open(bulk_log_file, 'w') as file:
               file.close()

          counter = 0

          for doc in cursor_mongo:

              doc_final = {}

              data_enc = doc["data"]
              data_dec = decode_now(data_enc)

              loaded_str = json.dumps([data_dec])
              loaded_str = loaded_str.replace("\x07", "")
              loaded_str = loaded_str.replace("\x08", "")
              loaded_str = loaded_str.replace("\\u0009", "")
              loaded_str = loaded_str.replace("\\u0008", "")
              loaded_str = loaded_str.replace("\\u0007", "")
              loaded_str = loaded_str.replace("\\u0006", "")
              loaded_str = loaded_str.replace("\\u0005", "")
              loaded_str = loaded_str.replace("\\u0004", "")
              loaded_str = loaded_str.replace("\\u0003", "")
              loaded_str = loaded_str.replace("\\u0002", "")
              loaded_str = loaded_str.replace("\\u0001", "")
              loaded_str = loaded_str.replace("\\u0000", "")
              loaded_str = loaded_str.replace("\\b", "")
              loaded_str = loaded_str.replace("\\", "")
              loaded_str = loaded_str.replace("nn", "")
              loaded_str = loaded_str.replace("\"\"", "\"")
              loaded_str = loaded_str.replace("\, ", "\,")
              loaded_str = loaded_str.replace("u000b", "")

              regex_str = re.findall(r"\:(.*?)\,", loaded_str)
              regex_str2 = re.findall(r"\"calc_angle\"\:(.*?)\}", loaded_str)

              regex_list = list(set(regex_str).union(regex_str2))

#              print regex_list

              for token in regex_list:
                  loaded_str = loaded_str.replace(token, '"' + token + '"')

#              print json.loads(json.dumps(loaded_str))
#              str_escape = json.loads(json.dumps([loaded_str]))
#              print str_escape

              doc_final["data"] = json.loads(json.dumps(loaded_str))
              doc_final["datetime"] = doc["datetime"]
              doc_final["hash"] = doc["hash"]
              doc_final["nonce"] = doc["nonce"]
              doc_final["merkle_root"] = doc["merkle_root"]

              block_id = str(doc["_id"])

              doc_final["block_id"] = block_id

              json_formatted = json.dumps(doc_final, indent=4, sort_keys=True)
              json_formatted = json_formatted.replace(r'\"{', '{')
              json_formatted = json_formatted.replace(r'\"[', '[')
              json_formatted = json_formatted.replace(r'\"]\"', ']')
              json_formatted = json_formatted.replace(r'\]"', ']')
              json_formatted = json_formatted.replace("\"]\"", ']')
              json_formatted = json_formatted.replace("\"[", '[')
              json_formatted = json_formatted.replace('\"', '"')
              json_formatted = json_formatted.replace('}\]', '}]')
              json_formatted = json_formatted.replace('\\"', '"')
              json_formatted = json_formatted.replace('""', '"')
              json_formatted = json_formatted.replace('\"\"', '\"')
              json_formatted = json_formatted.replace(', ', ',')
              json_formatted = json_formatted.replace('}"', '}')
              json_formatted = json_formatted.replace(']"', ']')
              json_formatted = json_formatted.replace('\\]', ']')
              json_formatted = json_formatted.replace('u0010', '')

#              print json_formatted
#              print "Processing document: " + block_id

              with open(bulk_log_file, "a") as logfile:
                 logfile.write("Processing document: " + block_id + "\n")

              logfile.close()

              sys.stdout.write("Processing document ID: \033[1;33;40m%s (%d/%d)\r\033[0m" % (block_id, counter, num_docs))
              sys.stdout.flush()

              filename = ouput_path + "/document_" + block_id + ".json"
              save_to_file(filename, json_formatted)

              counter = counter + 1

          print ""
          print "\033[1;37;40mAll data extracted from blockchain!!\033[0m"

      except Exception as e:
          cursor.show()
          pass
#          traceback.print_exc()
      except KeyboardInterrupt:
        cursor.show()
        print "\n"
        pass


if __name__ == "__main__":
    main()
    cursor.show()
    print ""
