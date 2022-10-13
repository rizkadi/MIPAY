import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.navigateToUrl('https://dev-payroll.its.ac.id/tpb/penerimaan-tpb')

WebUI.selectOptionByValue(findTestObject('Object Repository/TPB/rilis_bruto_tpb/pilih_bulan'), '3', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/TPB/rilis_bruto_tpb/select_Semua Data                          _064e6c'), 
    'perubahan-bruto-tpb', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/TPB/rilis_bruto_tpb/select_Semua Data                          _064e6c'), 
    'semua-data', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/TPB/rilis_bruto_tpb/select_Bulanan Reguler                     _f3a3bd'), 
    '7', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/TPB/rilis_bruto_tpb/select_Bulanan Reguler                     _f3a3bd'), 
    '9', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/TPB/rilis_bruto_tpb/select_Bulanan Reguler                     _f3a3bd'), 
    '1', true)

WebUI.click(findTestObject('Object Repository/TPB/rilis_bruto_tpb/button_Tampilkan'))

WebUI.click(findTestObject('Object Repository/TPB/rilis_bruto_tpb/input_Belum Tervalidasi_id_tpb_validasi'))

WebUI.click(findTestObject('Object Repository/TPB/rilis_bruto_tpb/input_Belum Tervalidasi_id_tpb_validasi'))

WebUI.doubleClick(findTestObject('Object Repository/TPB/rilis_bruto_tpb/i_Belum Tervalidasi_fa fa-edit'))

WebUI.setText(findTestObject('Object Repository/TPB/rilis_bruto_tpb/input_Indeks Rupiah_indeks-rupiah'), '1')

WebUI.setText(findTestObject('Object Repository/TPB/rilis_bruto_tpb/input_Pemotong Tunjangan_pemotong-tunjangan'), '1')

WebUI.setText(findTestObject('Object Repository/TPB/rilis_bruto_tpb/input_Kurang Bayar_kurang-bayar'), '1')

WebUI.setText(findTestObject('Object Repository/TPB/rilis_bruto_tpb/input_Penambah Tunjangan_penambah-tunjangan'), '1')

WebUI.setText(findTestObject('Object Repository/TPB/rilis_bruto_tpb/textarea_Keterangan_keterangan'), 'TES')

WebUI.click(findTestObject('Object Repository/TPB/rilis_bruto_tpb/button_Simpan'))

WebUI.selectOptionByValue(findTestObject('Object Repository/TPB/rilis_bruto_tpb/pilih_bulan'), '3', true)

WebUI.click(findTestObject('Object Repository/TPB/rilis_bruto_tpb/button_Tampilkan'))

WebUI.verifyElementText(findTestObject('Object Repository/TPB/rilis_bruto_tpb/td_1'), '1')

WebUI.click(findTestObject('Object Repository/TPB/rilis_bruto_tpb/i_Belum Tervalidasi_fa fa-edit'))

WebUI.setText(findTestObject('Object Repository/TPB/rilis_bruto_tpb/input_Indeks Rupiah_indeks-rupiah'), '12000')

WebUI.setText(findTestObject('Object Repository/TPB/rilis_bruto_tpb/input_Pemotong Tunjangan_pemotong-tunjangan'), '0')

WebUI.setText(findTestObject('Object Repository/TPB/rilis_bruto_tpb/input_Kurang Bayar_kurang-bayar'), '0')

WebUI.setText(findTestObject('Object Repository/TPB/rilis_bruto_tpb/input_Penambah Tunjangan_penambah-tunjangan'), '0')

WebUI.setText(findTestObject('Object Repository/TPB/rilis_bruto_tpb/textarea_Keterangan_keterangan'), 'nope')

WebUI.click(findTestObject('Object Repository/TPB/rilis_bruto_tpb/button_Simpan'))

WebUI.selectOptionByValue(findTestObject('Object Repository/TPB/rilis_bruto_tpb/pilih_bulan'), '3', true)

WebUI.click(findTestObject('Object Repository/TPB/rilis_bruto_tpb/button_Tampilkan'))

WebUI.verifyElementText(findTestObject('Object Repository/TPB/rilis_bruto_tpb/td_1'), '0')

