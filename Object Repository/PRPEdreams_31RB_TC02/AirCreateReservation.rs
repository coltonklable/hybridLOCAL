<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AirCreateReservation</name>
   <tag></tag>
   <elementGuidId>f5e29a15-dca6-430e-92ef-4c0772d85d7d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
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
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soap:Envelope xmlns:soap=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v50_0&quot; xmlns:common_v50_0=&quot;http://www.travelport.com/schema/common_v50_0&quot;>
   &lt;soap:Body>
      &lt;univ:AirCreateReservationReq RetrieveProviderReservationDetails=&quot;false&quot; RetainReservation=&quot;Both&quot; TraceId=&quot;d59fa653-c000-4a4a-a560-2d7057c762d8&quot; TargetBranch=&quot;P7113929&quot; AuthorizedBy=&quot;UAPITESTING&quot; Version=&quot;0&quot; xmlns:univ=&quot;http://www.travelport.com/schema/universal_v50_0&quot;>
         &lt;com:BillingPointOfSaleInfo OriginApplication=&quot;UAPI&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;/>
         &lt;com:BookingTraveler Key=&quot;1&quot; TravelerType=&quot;ADT&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;>
            &lt;com:BookingTravelerName Prefix=&quot;Mr&quot; First=&quot;Rajiv&quot; Last=&quot;Sharma&quot;/>
            &lt;com:PhoneNumber Key=&quot;1005359&quot; CountryCode=&quot;011&quot; Location=&quot;DEN&quot; Number=&quot;227-722-2454&quot; Extension=&quot;22&quot; AreaCode=&quot;222&quot; Type=&quot;Home&quot; Text=&quot;Abc-Xy&quot;/>
            &lt;com:Email Type=&quot;Home&quot; EmailID=&quot;jtestora@travelport.com&quot;/>
            &lt;com:SSR Key=&quot;1&quot; Type=&quot;DOCS&quot; Status=&quot;HK&quot; Carrier=&quot;${Carrier}&quot; FreeText=&quot;P/CA/F9850356/GB/04JAN80/M/01JAN14/Sharma/RajivMr&quot;/>
            &lt;com:Address>
               &lt;com:AddressName>Jan Restora&lt;/com:AddressName>
               &lt;com:Street>6901 S. Havana&lt;/com:Street>
               &lt;com:Street>Apt 3&lt;/com:Street>
               &lt;com:City>Englewood&lt;/com:City>
               &lt;com:State>CO&lt;/com:State>
               &lt;com:PostalCode>80111&lt;/com:PostalCode>
               &lt;com:Country>US&lt;/com:Country>
            &lt;/com:Address>
         &lt;/com:BookingTraveler>
       &lt;!--  &lt;com:BookingTraveler Key=&quot;2&quot; TravelerType=&quot;CNN&quot; DOB=&quot;2015-08-09&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;>
            &lt;com:BookingTravelerName Prefix=&quot;Mr&quot; First=&quot;Karan&quot; Last=&quot;Sharma&quot;/>
            &lt;com:PhoneNumber Key=&quot;1005360&quot; CountryCode=&quot;011&quot; Location=&quot;DEN&quot; Number=&quot;227-722-2454&quot; Extension=&quot;22&quot; AreaCode=&quot;222&quot; Type=&quot;Home&quot; Text=&quot;Abc-Xy&quot;/>
            &lt;com:Email Type=&quot;Home&quot; EmailID=&quot;jtestora@travelport.com&quot;/>
            &lt;com:SSR Key=&quot;2&quot; Type=&quot;DOCS&quot; Status=&quot;HK&quot; Carrier=&quot;${Carrier}&quot; FreeText=&quot;P/CA/F9850356/GB/04JAN80/M/01JAN14/Sharma/KaranMr&quot;/>
            
         &lt;/com:BookingTraveler>-->
         &lt;GeneralRemark UseProviderNativeMode=&quot;true&quot; TypeInGds=&quot;Basic&quot; xmlns=&quot;http://www.travelport.com/schema/common_v50_0&quot;>
            &lt;RemarkData>Booking 1&lt;/RemarkData>
         &lt;/GeneralRemark>
         &lt;GeneralRemark UseProviderNativeMode=&quot;true&quot; TypeInGds=&quot;Basic&quot; xmlns=&quot;http://www.travelport.com/schema/common_v50_0&quot;>
            &lt;RemarkData>Re- Booking 1&lt;/RemarkData>
         &lt;/GeneralRemark>
         &lt;com:ContinuityCheckOverride Key=&quot;1T&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;>true&lt;/com:ContinuityCheckOverride>
         &lt;com:FormOfPayment Key=&quot;PDz8y7xu4hGdeB/wYIhwmw==&quot; Type=&quot;Cash&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;/>
${APS}
        &lt;com:ActionStatus TicketDate=&quot;2021-12-01T23:59:00.000+10:00&quot; Type=&quot;ACTIVE&quot; ProviderCode=&quot;1G&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;/>
         &lt;com:Payment Key=&quot;qLIEbNPZQ5OnEfVRZKEb+w==&quot; Type=&quot;Itinerary&quot; FormOfPaymentRef=&quot;PDz8y7xu4hGdeB/wYIhwmw==&quot; Amount=&quot;AUD178.40&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;/>
      &lt;/univ:AirCreateReservationReq>
   &lt;/soap:Body>
&lt;/soap:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://WAS-UNIVERSALAPI-33890-100.UT.TVLPORT.COM:33890/kestrel/AirService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.APS</defaultValue>
      <description></description>
      <id>b1eec7c1-32c0-4413-9d33-f74741353d7a</id>
      <masked>false</masked>
      <name>APS</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TargetBranch</defaultValue>
      <description></description>
      <id>b29c9af1-91de-4662-9cdd-3a806bad2c8d</id>
      <masked>false</masked>
      <name>TargetBranch</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Carrier</defaultValue>
      <description></description>
      <id>04f34808-0a5d-449e-b3ea-a5adb38aed57</id>
      <masked>false</masked>
      <name>Carrier</name>
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
