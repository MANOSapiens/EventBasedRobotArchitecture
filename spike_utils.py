import os
import serial
import time
import tqdm
import base64
import ast

def wait_for_prompt(ser):
  buf = b''
  big_buf = b''
  start_time = time.time()
  elapsed = 0
  while elapsed < 1:
    c = ser.in_waiting
    ser.timeout = 1 - elapsed
    x = ser.read(c if c else 1)
    big_buf += x
    buf = (buf + x)[-5:]
    if buf == b'\n>>> ':
      return big_buf
    
    elapsed = time.time() - start_time
  

def init_connection(port, baudrate=115200):
  ser = serial.Serial(port, baudrate, timeout = 0.1)
  ser.write(b'\x03')
  wait_for_prompt(ser)
  return ser

def write_command(ser, cmd):
  ser.write(cmd + b'\r\n')
  response = wait_for_prompt(ser)

  r = response.decode('utf8').split('\r\n')[1:-1]
  if len(r) > 0:
    return ast.literal_eval(r[-1])

def upload_file(ser, file_path, loc):
    size = os.path.getsize(file_path)
    path, file = os.path.split(file_path)
    

    write_command(ser, b'')
    write_command(ser, b"import ubinascii")
    write_command(ser, b"f = open('%s/%s', 'wb')" % (loc.encode('utf8'), file.encode('utf8')))
    wait_for_prompt(ser)

    with tqdm.tqdm(total=size, unit='B', unit_scale=True) as pbar:
        with open(file_path, "rb") as f:
            byte = f.read(192)
            while len(byte) > 0:
                write_command(ser, b"f.write(ubinascii.a2b_base64('%s'))" % base64.b64encode(byte))
                pbar.update(len(byte))
                byte = f.read(192)
    write_command(ser, b"f.close()")

def download_file(ser, file, loc):
    write_command(ser, b"import ubinascii")
    write_command(ser, b"f = open('%s', 'rb')" % file.encode('utf8'))
    write_command(ser, b"content = f.read()")
    write_command(ser, b"f.close()")
    content = write_command(ser, b"ubinascii.b2a_base64(content)")
    with open(loc, "wb+") as f:
        f.write(base64.b64decode(content))

def create_dir(ser, dir):
    write_command(ser, b"import os")
    write_command(ser, b"os.mkdir('%s')" % dir.encode('utf8'))

def delete_file(ser, file):
    write_command(ser, b"import os")
    write_command(ser, b"os.remove('%s')" % file.encode('utf8'))

def delete_dir(ser, dir):
    write_command(ser, b"import os")
    write_command(ser, b"os.rmdir('%s')" % dir.encode('utf8'))

def list_files(ser, dir):
    write_command(ser, b"import os")
    return write_command(ser, b"print(os.listdir('%s'))" % dir.encode('utf8'))

def close_connection(ser):
    ser.write(b'\x04')
    ser.flush()