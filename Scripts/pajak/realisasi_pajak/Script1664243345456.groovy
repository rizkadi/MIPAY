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

WebUI.click(findTestObject('Object Repository/pajak/realisasi_pajak/span_Realisasi Pajak'))

WebUI.setText(findTestObject('Object Repository/pajak/realisasi_pajak/input'), 'solekan')

WebUI.verifyElementText(findTestObject('Object Repository/pajak/realisasi_pajak/td_Solekan'), 'Solekan')

WebUI.setText(findTestObject('Object Repository/pajak/realisasi_pajak/input'), '')

WebUI.selectOptionByValue(findTestObject('Object Repository/pajak/realisasi_pajak/select_102550100'), '25', true)

WebUI.verifyTextPresent('Menampilkan 1 sampai 25', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/pajak/realisasi_pajak/select_102550100'), '50', true)

WebUI.verifyTextPresent('Menampilkan 1 sampai 50', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/pajak/realisasi_pajak/select_102550100'), '100', true)

WebUI.verifyTextPresent('Menampilkan 1 sampai 100', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/pajak/realisasi_pajak/select_102550100'), '10', true)

WebUI.verifyTextPresent('Menampilkan 1 sampai 10', true)

