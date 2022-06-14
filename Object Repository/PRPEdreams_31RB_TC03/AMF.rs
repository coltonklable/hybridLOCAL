<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AMF</name>
   <tag></tag>
   <elementGuidId>0b068099-3228-4b07-a670-00e2a0c5c8a2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic dUFQSTkwNDQxOTkwNzctZjMyYWM2MTM6TmV3cGFzc180VSoyMA==</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
&lt;soapenv:Header/>
&lt;soapenv:Body>
&lt;universal:AirMerchandisingFulfillmentReq xmlns:universal=&quot;http://www.travelport.com/schema/universal_v51_0&quot; xmlns:common=&quot;http://www.travelport.com/schema/common_v51_0&quot;                   xmlns:common_v51_0=&quot;http://www.travelport.com/schema/common_v51_0&quot; xmlns:rail=&quot;http://www.travelport.com/schema/rail_v51_0&quot; xmlns:cruise=&quot;http://www.travelport.com/schema/cruise_v51_0&quot; xmlns:hotel=&quot;http://www.travelport.com/schema/hotel_v51_0&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v51_0&quot; xmlns:passive=&quot;http://www.travelport.com/schema/passive_v51_0&quot; xmlns:xsi=&quot;http://www.w3.org/2001/XMLSchema-instance&quot; xmlns:vehicle=&quot;http://www.travelport.com/schema/vehicle_v51_0&quot; AuthorizedBy=&quot;SVTTester&quot; TargetBranch=&quot;P7113929&quot;>
&lt;common:BillingPointOfSaleInfo OriginApplication=&quot;uAPI&quot;/>
&lt;air:HostReservation Carrier=&quot;${GlobalVariable.Carrier}&quot; CarrierLocatorCode=&quot;${GlobalVariable.CLC}&quot; ProviderCode=&quot;${GlobalVariable.ProviderCode}&quot; ProviderLocatorCode=&quot;${GlobalVariable.PNR}&quot; UniversalLocatorCode=&quot;${GlobalVariable.UR}&quot;/>
&lt;air:AirSolution>

${GlobalVariable.SegAMF}
&lt;/air:AirSolution>
&lt;common:CreditCard Key=&quot;1&quot; Type=&quot;MC&quot; Number=&quot;5200000000000007&quot; ExpDate=&quot;2021-12&quot; Name=&quot;JAYA KUMAR&quot; CVV=&quot;123&quot;>
&lt;common:BillingAddress>
&lt;common:AddressName>Sandy Parent&lt;/common:AddressName>
&lt;common:Street>Apt 2&lt;/common:Street>
&lt;common:City>Englewood&lt;/common:City>
&lt;common:State>CO&lt;/common:State>
&lt;common:PostalCode>80111&lt;/common:PostalCode>
&lt;common:Country>US&lt;/common:Country>
&lt;/common:BillingAddress>
&lt;/common:CreditCard>
&lt;air:OptionalServices>
${GlobalVariable.OPS}
&lt;/air:OptionalServices>
&lt;/universal:AirMerchandisingFulfillmentReq>
&lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://WAS-UNIVERSALAPI-33890-100.UT.TVLPORT.COM:33890/kestrel/AirService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.RecordLocator</defaultValue>
      <description></description>
      <id>b1eec7c1-32c0-4413-9d33-f74741353d7a</id>
      <masked>false</masked>
      <name>AirRecordLocator</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.SegAMF</defaultValue>
      <description></description>
      <id>a0aa52ba-2ce6-470f-94ae-3cb7dc5e4c44</id>
      <masked>false</masked>
      <name>SegAMF</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager as WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyElementText(response, 'LowFareSearchRsp.AirSegmentList.AirSegment','')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
