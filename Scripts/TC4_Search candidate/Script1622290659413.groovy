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

WebUI.navigateToUrl('https://hajarherrou-trials71.orangehrmlive.com/client/#/noncore/recruitment/viewCandidates')

WebUI.click(findTestObject('Object Repository/Search candidate/Page_OrangeHRM/i_search'))

WebUI.click(findTestObject('Object Repository/Search candidate/Page_OrangeHRM/div_Help_textarea_candidateSearch_candidate'))

WebUI.setText(findTestObject('Object Repository/Search candidate/Page_OrangeHRM/input_Help_candidate-search candidate-multi_c2a75d'), 
    'fatima')

WebUI.click(findTestObject('Object Repository/Search candidate/Page_OrangeHRM/p_Fatima Zahrae Sen'))

WebUI.click(findTestObject('Object Repository/Search candidate/Page_OrangeHRM/div_Job Title_textarea_candidateSearch_jobVacancy'))

WebUI.setText(findTestObject('Object Repository/Search candidate/Page_OrangeHRM/input_Job Title_employee-search validate'), 
    'ux')

WebUI.click(findTestObject('Object Repository/Search candidate/Page_OrangeHRM/p_UIUX Engineer'))

WebUI.click(findTestObject('Object Repository/Search candidate/Page_OrangeHRM/div_SaturdayMay292021JanuaryFebruaryMarchAp_8128ce'))

WebUI.click(findTestObject('Object Repository/Search candidate/Page_OrangeHRM/div_14'))

WebUI.click(findTestObject('Object Repository/Search candidate/Page_OrangeHRM/div_SaturdayMay292021JanuaryFebruaryMarchAp_8128ce_1'))

WebUI.click(findTestObject('Object Repository/Search candidate/Page_OrangeHRM/div_21'))

WebUI.click(findTestObject('Object Repository/Search candidate/Page_OrangeHRM/a_Search'))

