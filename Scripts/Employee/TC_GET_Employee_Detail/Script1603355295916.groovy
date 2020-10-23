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

response = WS.sendRequestAndVerify(findTestObject('Employee/GET_Employee_Detail'))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'data.firstName', "Kijang")

WS.verifyElementPropertyValue(response, 'data.middleName', "")

WS.verifyElementPropertyValue(response, 'data.lastName', "Satu")

WS.verifyElementPropertyValue(response, 'data.code', "0001")

WS.verifyElementPropertyValue(response, 'data.employeeId', "1")

WS.verifyElementPropertyValue(response, 'data.fullName', "Kijang Satu")

WS.verifyElementPropertyValue(response, 'data.status', Probation)

WS.verifyElementPropertyValue(response, 'data.dob', null)

WS.verifyElementPropertyValue(response, 'data.driversLicenseNumber', "")

WS.verifyElementPropertyValue(response, 'data.licenseExpiryDate', null)

WS.verifyElementPropertyValue(response, 'data.maritalStatus', null)

WS.verifyElementPropertyValue(response, 'data.gender', null)

WS.verifyElementPropertyValue(response, 'data.otherId', "")

WS.verifyElementPropertyValue(response, 'data.nationality', null)

WS.verifyElementPropertyValue(response, 'data.unit', null)

WS.verifyElementPropertyValue(response, 'data.jobTitle', 'QA Engineer')

WS.verifyElementPropertyValue(response, 'data.supervisor', null)