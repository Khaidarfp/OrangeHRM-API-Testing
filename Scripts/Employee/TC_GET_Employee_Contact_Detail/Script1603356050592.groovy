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
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

response = WS.sendRequestAndVerify(findTestObject('Employee/GET_Employee_Contact_Detail'))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'data.id', '1')

WS.verifyElementPropertyValue(response, 'data.code', '0001')

WS.verifyElementPropertyValue(response, 'data.fullName', 'Kijang Satu')

WS.verifyElementPropertyValue(response, 'data.addressStreet1', 'Jl. Street')

WS.verifyElementPropertyValue(response, 'data.addressStreet2', 'Jalanan No.21')

WS.verifyElementPropertyValue(response, 'data.city', 'Tangerang')

WS.verifyElementPropertyValue(response, 'data.state', 'Indonesia')

WS.verifyElementPropertyValue(response, 'data.zip', '14141')

WS.verifyElementPropertyValue(response, 'data.county', 'INDONESIA')

WS.verifyElementPropertyValue(response, 'data.homeTelephone', '02155443322')

WS.verifyElementPropertyValue(response, 'data.workTelephone', '08123456789')

WS.verifyElementPropertyValue(response, 'data.mobile', '08123456789')

WS.verifyElementPropertyValue(response, 'data.workEmail', 'kijang@qa.cilsy.id')

WS.verifyElementPropertyValue(response, 'data.otherEmail', 'kijang@cilsy.id')

