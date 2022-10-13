<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Beranda            Pegawai            _aadc5a</name>
   <tag></tag>
   <elementGuidId>d1cee903-19c0-49bb-8836-1fcb5f69ab97</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>1</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>de350295-1b4c-429a-8a26-1a4a086d7ffc</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
        
            
            
                                    
                            
            
                
                
            
        
        
            
                

    
        
        Beranda
    


    

    Pegawai



    
        
        Biodata Pegawai
    



    
        
        Riwayat Keaktifan
    





    
        
        Daftar Pegawai
    



    
        
        Daftar Ajuan Rekening
    





    
        
        Daftar Keaktifan Pegawai
    





    Take Home Pay


    
        
        Rincian Pendapatan
    


    
        
        Rincian Pot. Pendapatan
    


    
        
        Bukti Potong Pajak
    


    
        
        SPT Tahunan
    


    
        
        Slip Gaji Pegawai
    



    
        
        Daftar Ajuan Slip Gaji
    



    
        
        Daftar Pendapatan Pegawai
    


    Pajak

    
        
        Perencanaan Pajak
    



    
        
        Realisasi Pajak
    



    TPB




    
        
        Rilis Bruto TPB
    





    
        
        Rilis Final TPB
    



    Remunerasi




    
        
        Rilis Bruto Remunerasi
    





    
        
        Rilis Final Remunerasi
    



    Honor



    
        
        Daftar Jenis Honor
    



    
        
        Daftar Penerima Honor
    



    
    
        Rilis Bruto Honor
    




    
        
        Rilis Final Honor
    


            
        
        
        
            
                
                                            
                            
                        
                                    
                                
    
        
            
                
            
        
        
            
                
            
            
                Rizky Adi Putra
            
            Super Administrator
            Direktorat Pengembangan Teknologi dan Sistem Informasi
            
                            
                    Mode gelap
                
                        
                Ganti hak akses
            
            
                Keluar
            
        
    

            
            
                
                    
                        
                                                        
                                                            
                        
                        
                        
                        
                                                    
                    

                    
                        
    
        
            Beranda > TPB > Data Penerimaan
        
    
    
        
            Edit Data Penerimaan TPB
            
                
                    
                        Nama
                        :
                        A.A. BGS. Dinariyana Dwi Putranta, S.T., MES., Ph.D.
                    
                    
                        NIP
                        :
                        197505102000031001
                    
                
                
                    
                        Jabatan
                        :
                        Kepala Program Studi Pascasarjana Inovasi Sistem dan Teknologi
                    
                    
                        Unit
                        :
                        Institut Teknologi Sepuluh Nopember
                    
                
            
        
    

    
        
            
                
                    
                                              
                        Tahun
                        
                          
                        
                      

                      
                        Bulan
                        
                          
                        
                      

                      
                        Nilai Jabatan
                                            
                          
                        
                      

                      
                        Indeks Rupiah
                        
                          
                        
                      

                      
                        Faktor Pengali
                        
                          
                        
                      

                      
                        Indeks TPB
                        
                          
                        
                      

                      
                        Pemotong Tunjangan
                        
                          
                        
                      

                      
                        Kurang Bayar
                        
                          
                        
                      

                      
                        Penambah Tunjangan
                        
                          
                        
                      

                      
                        Bruto TPB Lama
                        
                          
                        
                      

                      
                        Bruto TPB Baru
                        
                          
                        
                      

                      
                        Keterangan
                        
                          
                        
                      

                      

                      
                        
                            
                              Batal
                              Simpan
                            
                        
                      

                    
                
            
        
    


                    
                
                
                    
                        
                            
                                Copyright © document.write(new Date().getFullYear())2022 Institut Teknologi Sepuluh Nopember
                            
                            
                                                                    
                                                            
                        
                    
                
                
                    
                        
                            
                                Copyright © document.write(new Date().getFullYear())2022 ITS
                            
                            
                                                                    
                                                            
                        
                    
                
            
        

        
    
        
            
                
                    ×
                
                Hak Akses
                Hak akses Anda saat ini: Super Administrator Direktorat Pengembangan Teknologi dan Sistem Informasi.
                
                                        
                        
                            Pilih hak akses...
                                                            
                                    Pegawai
                                                                    
                                                            
                                    Super Administrator
                                                                            Direktorat Pengembangan Teknologi dan Sistem Informasi
                                        
                                                                    
                                                            
                                    Administrator
                                                                            Direktorat Pengembangan Teknologi dan Sistem Informasi
                                        
                                                                    
                                                    
                    
                    
                        Ganti
                        Batal
                    
                
            
        
    


        
        
        
        

        
        

        
    $(document).ready( function () {
      $(&quot;.uang&quot;).mask(
        '#.##0', 
        {
          reverse: true,
          translation: {
            '#': {
              pattern: /-|\d/,
              recursive: true
            }
          },
          onChange: function(value, e) {
            var target = e.target,
                position = target.selectionStart; // Capture initial position

            target.value = value.replace(/(?!^)-/g, '').replace(/^\./, '').replace(/^-\./, '-');

            target.selectionEnd = position; // Set the cursor back to the initial position.
          }
        }
      );

      $(&quot;.uang-positif&quot;).mask('#.##0', {reverse: true});

      $(&quot;#indeks-rupiah, #pemotong-tunjangan, #kurang-bayar, #penambah-tunjangan&quot;).on(&quot;keyup&quot;, function(e){
        var indeks_rupiah  = $(&quot;#indeks-rupiah&quot;).val().replace(/\./g,'');
        var nilai_jabatan  = $(&quot;#nilai-jabatan&quot;).val().replace(/\./g,'');
        var faktor_pengali = $(&quot;#faktor-pengali&quot;).val().replace(/\./g,'');
        var pemotong_tunjangan = $(&quot;#pemotong-tunjangan&quot;).val().replace(/\./g,'');
        var kurang_bayar       = $(&quot;#kurang-bayar&quot;).val().replace(/\./g,'');
        var penambah_tunjangan = $(&quot;#penambah-tunjangan&quot;).val().replace(/\./g,'');
        
        var indeks_tpb = parseInt(indeks_rupiah) * parseInt(nilai_jabatan) * parseInt(faktor_pengali);
        var bruto      = parseInt(indeks_tpb) - parseInt(pemotong_tunjangan) + parseInt(kurang_bayar) + parseInt(penambah_tunjangan);

        $(&quot;#indeks-tpb-baru&quot;).val(indeks_tpb).trigger(&quot;input&quot;);
        $(&quot;#bruto-tpb-baru&quot;).val(bruto).trigger(&quot;input&quot;);

      });
    });


        
            $(document).ready(function($) {
                $(&quot;.custom-file-input&quot;).on(&quot;change&quot;, function() {
                    var fileName = $(this).val().split(&quot;\\&quot;).pop();
                    $(this).siblings(&quot;.custom-file-label&quot;).addClass(&quot;selected&quot;).html(fileName);
                });
            });
        

    

id(&quot;sideMenu&quot;)/li[@class=&quot;nav-item&quot;]/a[@class=&quot;nav-link&quot;]/span[1]</value>
      <webElementGuid>2eaa88da-bfcf-4103-80e7-04961f3b7b2b</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>557087c4-e4fe-4506-84fb-01fa5dcc6843</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>1c628e2a-0883-496c-a716-261c97ac829a</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
        
            
            
                                    
                            
            
                
                
            
        
        
            
                

    
        
        Beranda
    


    

    Pegawai



    
        
        Biodata Pegawai
    



    
        
        Riwayat Keaktifan
    





    
        
        Daftar Pegawai
    



    
        
        Daftar Ajuan Rekening
    





    
        
        Daftar Keaktifan Pegawai
    





    Take Home Pay


    
        
        Rincian Pendapatan
    


    
        
        Rincian Pot. Pendapatan
    


    
        
        Bukti Potong Pajak
    


    
        
        SPT Tahunan
    


    
        
        Slip Gaji Pegawai
    



    
        
        Daftar Ajuan Slip Gaji
    



    
        
        Daftar Pendapatan Pegawai
    


    Pajak

    
        
        Perencanaan Pajak
    



    
        
        Realisasi Pajak
    



    TPB




    
        
        Rilis Bruto TPB
    





    
        
        Rilis Final TPB
    



    Remunerasi




    
        
        Rilis Bruto Remunerasi
    





    
        
        Rilis Final Remunerasi
    



    Honor



    
        
        Daftar Jenis Honor
    



    
        
        Daftar Penerima Honor
    



    
    
        Rilis Bruto Honor
    




    
        
        Rilis Final Honor
    


            
        
        
        
            
                
                                            
                            
                        
                                    
                                
    
        
            
                
            
        
        
            
                
            
            
                Rizky Adi Putra
            
            Super Administrator
            Direktorat Pengembangan Teknologi dan Sistem Informasi
            
                            
                    Mode gelap
                
                        
                Ganti hak akses
            
            
                Keluar
            
        
    

            
            
                
                    
                        
                                                        
                                                            
                        
                        
                        
                        
                                                    
                    

                    
                        
    
        
            Beranda > TPB > Data Penerimaan
        
    
    
        
            Edit Data Penerimaan TPB
            
                
                    
                        Nama
                        :
                        A.A. BGS. Dinariyana Dwi Putranta, S.T., MES., Ph.D.
                    
                    
                        NIP
                        :
                        197505102000031001
                    
                
                
                    
                        Jabatan
                        :
                        Kepala Program Studi Pascasarjana Inovasi Sistem dan Teknologi
                    
                    
                        Unit
                        :
                        Institut Teknologi Sepuluh Nopember
                    
                
            
        
    

    
        
            
                
                    
                                              
                        Tahun
                        
                          
                        
                      

                      
                        Bulan
                        
                          
                        
                      

                      
                        Nilai Jabatan
                                            
                          
                        
                      

                      
                        Indeks Rupiah
                        
                          
                        
                      

                      
                        Faktor Pengali
                        
                          
                        
                      

                      
                        Indeks TPB
                        
                          
                        
                      

                      
                        Pemotong Tunjangan
                        
                          
                        
                      

                      
                        Kurang Bayar
                        
                          
                        
                      

                      
                        Penambah Tunjangan
                        
                          
                        
                      

                      
                        Bruto TPB Lama
                        
                          
                        
                      

                      
                        Bruto TPB Baru
                        
                          
                        
                      

                      
                        Keterangan
                        
                          
                        
                      

                      

                      
                        
                            
                              Batal
                              Simpan
                            
                        
                      

                    
                
            
        
    


                    
                
                
                    
                        
                            
                                Copyright © document.write(new Date().getFullYear())2022 Institut Teknologi Sepuluh Nopember
                            
                            
                                                                    
                                                            
                        
                    
                
                
                    
                        
                            
                                Copyright © document.write(new Date().getFullYear())2022 ITS
                            
                            
                                                                    
                                                            
                        
                    
                
            
        

        
    
        
            
                
                    ×
                
                Hak Akses
                Hak akses Anda saat ini: Super Administrator Direktorat Pengembangan Teknologi dan Sistem Informasi.
                
                                        
                        
                            Pilih hak akses...
                                                            
                                    Pegawai
                                                                    
                                                            
                                    Super Administrator
                                                                            Direktorat Pengembangan Teknologi dan Sistem Informasi
                                        
                                                                    
                                                            
                                    Administrator
                                                                            Direktorat Pengembangan Teknologi dan Sistem Informasi
                                        
                                                                    
                                                    
                    
                    
                        Ganti
                        Batal
                    
                
            
        
    


        
        
        
        

        
        

        
    $(document).ready( function () {
      $(&quot;.uang&quot;).mask(
        &quot; , &quot;'&quot; , &quot;#.##0&quot; , &quot;'&quot; , &quot;, 
        {
          reverse: true,
          translation: {
            &quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;: {
              pattern: /-|\d/,
              recursive: true
            }
          },
          onChange: function(value, e) {
            var target = e.target,
                position = target.selectionStart; // Capture initial position

            target.value = value.replace(/(?!^)-/g, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).replace(/^\./, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).replace(/^-\./, &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;);

            target.selectionEnd = position; // Set the cursor back to the initial position.
          }
        }
      );

      $(&quot;.uang-positif&quot;).mask(&quot; , &quot;'&quot; , &quot;#.##0&quot; , &quot;'&quot; , &quot;, {reverse: true});

      $(&quot;#indeks-rupiah, #pemotong-tunjangan, #kurang-bayar, #penambah-tunjangan&quot;).on(&quot;keyup&quot;, function(e){
        var indeks_rupiah  = $(&quot;#indeks-rupiah&quot;).val().replace(/\./g,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        var nilai_jabatan  = $(&quot;#nilai-jabatan&quot;).val().replace(/\./g,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        var faktor_pengali = $(&quot;#faktor-pengali&quot;).val().replace(/\./g,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        var pemotong_tunjangan = $(&quot;#pemotong-tunjangan&quot;).val().replace(/\./g,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        var kurang_bayar       = $(&quot;#kurang-bayar&quot;).val().replace(/\./g,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        var penambah_tunjangan = $(&quot;#penambah-tunjangan&quot;).val().replace(/\./g,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        
        var indeks_tpb = parseInt(indeks_rupiah) * parseInt(nilai_jabatan) * parseInt(faktor_pengali);
        var bruto      = parseInt(indeks_tpb) - parseInt(pemotong_tunjangan) + parseInt(kurang_bayar) + parseInt(penambah_tunjangan);

        $(&quot;#indeks-tpb-baru&quot;).val(indeks_tpb).trigger(&quot;input&quot;);
        $(&quot;#bruto-tpb-baru&quot;).val(bruto).trigger(&quot;input&quot;);

      });
    });


        
            $(document).ready(function($) {
                $(&quot;.custom-file-input&quot;).on(&quot;change&quot;, function() {
                    var fileName = $(this).val().split(&quot;\\&quot;).pop();
                    $(this).siblings(&quot;.custom-file-label&quot;).addClass(&quot;selected&quot;).html(fileName);
                });
            });
        

    

id(&quot;sideMenu&quot;)/li[@class=&quot;nav-item&quot;]/a[@class=&quot;nav-link&quot;]/span[1]&quot;) or . = concat(&quot;
        
            
            
                                    
                            
            
                
                
            
        
        
            
                

    
        
        Beranda
    


    

    Pegawai



    
        
        Biodata Pegawai
    



    
        
        Riwayat Keaktifan
    





    
        
        Daftar Pegawai
    



    
        
        Daftar Ajuan Rekening
    





    
        
        Daftar Keaktifan Pegawai
    





    Take Home Pay


    
        
        Rincian Pendapatan
    


    
        
        Rincian Pot. Pendapatan
    


    
        
        Bukti Potong Pajak
    


    
        
        SPT Tahunan
    


    
        
        Slip Gaji Pegawai
    



    
        
        Daftar Ajuan Slip Gaji
    



    
        
        Daftar Pendapatan Pegawai
    


    Pajak

    
        
        Perencanaan Pajak
    



    
        
        Realisasi Pajak
    



    TPB




    
        
        Rilis Bruto TPB
    





    
        
        Rilis Final TPB
    



    Remunerasi




    
        
        Rilis Bruto Remunerasi
    





    
        
        Rilis Final Remunerasi
    



    Honor



    
        
        Daftar Jenis Honor
    



    
        
        Daftar Penerima Honor
    



    
    
        Rilis Bruto Honor
    




    
        
        Rilis Final Honor
    


            
        
        
        
            
                
                                            
                            
                        
                                    
                                
    
        
            
                
            
        
        
            
                
            
            
                Rizky Adi Putra
            
            Super Administrator
            Direktorat Pengembangan Teknologi dan Sistem Informasi
            
                            
                    Mode gelap
                
                        
                Ganti hak akses
            
            
                Keluar
            
        
    

            
            
                
                    
                        
                                                        
                                                            
                        
                        
                        
                        
                                                    
                    

                    
                        
    
        
            Beranda > TPB > Data Penerimaan
        
    
    
        
            Edit Data Penerimaan TPB
            
                
                    
                        Nama
                        :
                        A.A. BGS. Dinariyana Dwi Putranta, S.T., MES., Ph.D.
                    
                    
                        NIP
                        :
                        197505102000031001
                    
                
                
                    
                        Jabatan
                        :
                        Kepala Program Studi Pascasarjana Inovasi Sistem dan Teknologi
                    
                    
                        Unit
                        :
                        Institut Teknologi Sepuluh Nopember
                    
                
            
        
    

    
        
            
                
                    
                                              
                        Tahun
                        
                          
                        
                      

                      
                        Bulan
                        
                          
                        
                      

                      
                        Nilai Jabatan
                                            
                          
                        
                      

                      
                        Indeks Rupiah
                        
                          
                        
                      

                      
                        Faktor Pengali
                        
                          
                        
                      

                      
                        Indeks TPB
                        
                          
                        
                      

                      
                        Pemotong Tunjangan
                        
                          
                        
                      

                      
                        Kurang Bayar
                        
                          
                        
                      

                      
                        Penambah Tunjangan
                        
                          
                        
                      

                      
                        Bruto TPB Lama
                        
                          
                        
                      

                      
                        Bruto TPB Baru
                        
                          
                        
                      

                      
                        Keterangan
                        
                          
                        
                      

                      

                      
                        
                            
                              Batal
                              Simpan
                            
                        
                      

                    
                
            
        
    


                    
                
                
                    
                        
                            
                                Copyright © document.write(new Date().getFullYear())2022 Institut Teknologi Sepuluh Nopember
                            
                            
                                                                    
                                                            
                        
                    
                
                
                    
                        
                            
                                Copyright © document.write(new Date().getFullYear())2022 ITS
                            
                            
                                                                    
                                                            
                        
                    
                
            
        

        
    
        
            
                
                    ×
                
                Hak Akses
                Hak akses Anda saat ini: Super Administrator Direktorat Pengembangan Teknologi dan Sistem Informasi.
                
                                        
                        
                            Pilih hak akses...
                                                            
                                    Pegawai
                                                                    
                                                            
                                    Super Administrator
                                                                            Direktorat Pengembangan Teknologi dan Sistem Informasi
                                        
                                                                    
                                                            
                                    Administrator
                                                                            Direktorat Pengembangan Teknologi dan Sistem Informasi
                                        
                                                                    
                                                    
                    
                    
                        Ganti
                        Batal
                    
                
            
        
    


        
        
        
        

        
        

        
    $(document).ready( function () {
      $(&quot;.uang&quot;).mask(
        &quot; , &quot;'&quot; , &quot;#.##0&quot; , &quot;'&quot; , &quot;, 
        {
          reverse: true,
          translation: {
            &quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;: {
              pattern: /-|\d/,
              recursive: true
            }
          },
          onChange: function(value, e) {
            var target = e.target,
                position = target.selectionStart; // Capture initial position

            target.value = value.replace(/(?!^)-/g, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).replace(/^\./, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;).replace(/^-\./, &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;);

            target.selectionEnd = position; // Set the cursor back to the initial position.
          }
        }
      );

      $(&quot;.uang-positif&quot;).mask(&quot; , &quot;'&quot; , &quot;#.##0&quot; , &quot;'&quot; , &quot;, {reverse: true});

      $(&quot;#indeks-rupiah, #pemotong-tunjangan, #kurang-bayar, #penambah-tunjangan&quot;).on(&quot;keyup&quot;, function(e){
        var indeks_rupiah  = $(&quot;#indeks-rupiah&quot;).val().replace(/\./g,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        var nilai_jabatan  = $(&quot;#nilai-jabatan&quot;).val().replace(/\./g,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        var faktor_pengali = $(&quot;#faktor-pengali&quot;).val().replace(/\./g,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        var pemotong_tunjangan = $(&quot;#pemotong-tunjangan&quot;).val().replace(/\./g,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        var kurang_bayar       = $(&quot;#kurang-bayar&quot;).val().replace(/\./g,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        var penambah_tunjangan = $(&quot;#penambah-tunjangan&quot;).val().replace(/\./g,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        
        var indeks_tpb = parseInt(indeks_rupiah) * parseInt(nilai_jabatan) * parseInt(faktor_pengali);
        var bruto      = parseInt(indeks_tpb) - parseInt(pemotong_tunjangan) + parseInt(kurang_bayar) + parseInt(penambah_tunjangan);

        $(&quot;#indeks-tpb-baru&quot;).val(indeks_tpb).trigger(&quot;input&quot;);
        $(&quot;#bruto-tpb-baru&quot;).val(bruto).trigger(&quot;input&quot;);

      });
    });


        
            $(document).ready(function($) {
                $(&quot;.custom-file-input&quot;).on(&quot;change&quot;, function() {
                    var fileName = $(this).val().split(&quot;\\&quot;).pop();
                    $(this).siblings(&quot;.custom-file-label&quot;).addClass(&quot;selected&quot;).html(fileName);
                });
            });
        

    

id(&quot;sideMenu&quot;)/li[@class=&quot;nav-item&quot;]/a[@class=&quot;nav-link&quot;]/span[1]&quot;))]</value>
      <webElementGuid>fc526a0c-8c4d-4019-8bb8-c5f526942455</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
