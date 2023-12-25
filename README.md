## Files management canister

### Deployment

```bash
dfx start --background --clean
npm run gen-deploy
```

### Create new folder
```bash
dfx canister call google_drive_backend create_folder '(
  record {
    folder_name = "Dacade Folder";
  }
)'
```

### Update folder info
```bash
dfx canister call google_drive_backend update_folder '(
  0,
  record {
    folder_name = "Dacade Folder Updated";
  }
)'
```

### Read all folder info
```bash
dfx canister call google_drive_backend get_all_folders
```

### Find folder by folder'id
```bash
dfx canister call google_drive_backend get_folder '(0)'
```

### Find folder by folder's name
```bash
dfx canister call google_drive_backend get_folder_by_name '("Dacade Folder Updated")'
```

### Create new file
```bash
dfx canister call google_drive_backend create_file '(
  record {
    folder_id = 0;
    file_name = "Learn Rust";
    mime_type = "pdf";
    content = "File content";
  }
)'
```

### Update file
```bash
dfx canister call google_drive_backend update_file '(
  1,
  record {
    folder_id = 0;
    file_name = "Learn Rust pt2";
    mime_type = "pdf";
    content = "File content updated";
  }
)'
```

### Read all files info
```bash
dfx canister call google_drive_backend get_all_files
```

### Find file info by its id
```bash
dfx canister call google_drive_backend get_file '(1)'
```

### Find all files by folder id
```bash
dfx canister call google_drive_backend get_all_files_by_folder_id '(0)'
```

### Find all files by folder name
```bash
dfx canister call google_drive_backend get_all_files_by_folder_name '("Dacade Folder Updated")'
```

### Delete file by its id
```bash
dfx canister call google_drive_backend delete_file '(1)'
```

### Stop dfx
```bash
dfx stop
```