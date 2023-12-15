# API Wilayah Indonesia berdasarkan data BPS tahun 2022

sumber: https://sig.bps.go.id/bridging-kode/index

# API Format Get Requests

```md
# type = bps -> 00 00 000 000

PROV kabkota Kecamatan Desa
00 00 000 000

# type = kemendagri -> 00.00.00.0000

PROV kabkota Kecamatan Desa
00 00 00 0000

# menampilkan semua profinsi

someapi.com/wilayah

# menampilkan list kabupaten/kota di profinsi

someapi.com/wilayah/<type>/<provinsi_id>

# menampilak list kecamatan

someapi.com/wilayah/<type>/<provinsi_id>/<kabkota_id>

# menampilak list desa

someapi.com/wilayah/<type>/<provinsi_id>/<kabkota_id>/<kecamatan_id>
```

# Example Usage

`sompeapi.com/wilayah/bps/32/01/001/010`
