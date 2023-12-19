import pysftp, paramiko
from base64 import decodebytes

def updateFiles():
    keydata = b"""AAAAB3NzaC1yc2EAAAADAQABAAABAQDs5IdQMNpJh6Mkzd+AiAFJ7PpyVQA5UDWWbAj39QwgvQJyoB3b0oQvyoX8hl3hQ9vrXlHbr7jhde+p/qPrEcMshidaFSzt+A5OMFPlfOB55drc4geGRm5q1jHqzdqCCoDT5rVhnTnF881BUfVyRovNLiZBMSGRKAUurYL91H3AQsZQhJNZ7Mp4oFFddcOpDacPRnobEulbXuGhpBr8UOGlWE+sSfJhybWDkFf9cSEh3ZLwVfvC770KFZHw6jPjy/gvHwXvwSTU0zTWiAgP+zEyQkpDDU2jrykOk6kQFnR9zy5BmD5GC8QCvAKbq3tAYTicf5GMKakAgBiY7yMcGraF"""
    key = paramiko.RSAKey(data=decodebytes(keydata))
    cnopts = pysftp.CnOpts()
    cnopts.hostkeys.add('ev3dev.local', 'ssh-rsa', key)
    
    with pysftp.Connection('ev3dev.local', username='robot', password='maker', cnopts=cnopts) as sftp:
        # Upload the file target/armv5te-unknown-linux-musleabi/release/EventBasedRobotArchitecture to /home/robot
        print('STARTING UPLOAD')
        sftp.put('target/armv5te-unknown-linux-musleabi/release/EventBasedRobotArchitecture', '/home/robot')
        print('COMPLETED UPLOAD')

        print('\n\n')
        print('========================================')
        #colorful print
        print('\033[1;32;40m' + 'The backend was updated!' + '\033[0m')
        print('========================================')            

    

if __name__ == '__main__':
    updateFiles()
