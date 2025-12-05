


# DIESEL 
`up.sql` should have the code related to applying any changes (eg: creating tables).  
`dowm.sql` this has code for reverting changes

```bash 
diesel migration generate customer 
```
this command creates a migration directory


```bash 
diesel migration run
```
this command runs the sql code in the up.sql file in the migrations folder 
