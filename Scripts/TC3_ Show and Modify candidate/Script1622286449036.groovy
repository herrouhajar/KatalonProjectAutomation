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

WebUI.switchToWindowTitle('OrangeHRM')

WebUI.verifyElementPresent(findTestObject('Details candidate/Page_OrangeHRM/td_Fatima Zahrae  Sen'), 1)

WebUI.click(findTestObject('Object Repository/Details candidate/Page_OrangeHRM/td_Fatima Zahrae  Sen'))

WebUI.switchToWindowIndex(1)

WebUI.click(findTestObject('Object Repository/Details candidate/Page_OrangeHRM/div_UIUX Engineer'))

WebUI.click(findTestObject('Object Repository/Details candidate/Page_OrangeHRM/p_DevOps Engineer'))

WebUI.setText(findTestObject('Object Repository/Details candidate/Page_OrangeHRM/input__addCandidateemail'), email)

WebUI.click(findTestObject('Object Repository/Details candidate/Page_OrangeHRM/a_Save'))

WebUI.click(findTestObject('Object Repository/Details candidate/Page_OrangeHRM/a_Yes, Continue'))

WebUI.click(findTestObject('Object Repository/Details candidate/Page_OrangeHRM/div_Successfully Saved'))

