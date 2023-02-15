import time
choice = False
try:
    if choice:
        while True:
            pass
    else:
        print("Please pull the plug on your computer sometime soon...")
        time.sleep(60 * 60 * 24 )

except KeyboardInterrupt:
    print("Thank you for breaking me loose :)")
finally:
    print("Finally ...")