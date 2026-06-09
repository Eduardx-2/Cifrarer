import shutil 


def data_disk():
    space, libsp,datasp = shutil.disk_usage("/")
    data = 1024.0*1024*1024
    return {
        "space": space/data,
        "libre": libsp/data,
        "dispo": datasp/data
    }
