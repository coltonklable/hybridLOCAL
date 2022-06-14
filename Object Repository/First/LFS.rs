<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>LFS</name>
   <tag></tag>
   <elementGuidId>4c9077c4-0c60-446b-8587-e233c3ef8adc</elementGuidId>
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
      <value>Basic dUFQSTg1MDE0OTQ0ODctZDcxMTBhMmE6TmV3cGFzc180VSoyMA==</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soap:Envelope xmlns:soap=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
	&lt;soap:Body>
		&lt;air:LowFareSearchReq TargetBranch=&quot;${GlobalVariable.TargetBranch}&quot; TraceId=&quot;056b592c-dea3-4450-92a5-a90288163f2f&quot; AuthorizedBy=&quot;UAPITesting&quot; SolutionResult=&quot;true&quot; xmlns:air=&quot;http://www.travelport.com/schema/air_v50_0&quot; xmlns:com=&quot;http://www.travelport.com/schema/common_v50_0&quot;>
			&lt;com:BillingPointOfSaleInfo OriginApplication=&quot;UAPI&quot;/>
			&lt;air:SearchAirLeg>
				&lt;air:SearchOrigin>
					&lt;com:CityOrAirport Code=&quot;${ORG}&quot;/>
				&lt;/air:SearchOrigin>
				&lt;air:SearchDestination>
					&lt;com:CityOrAirport Code=&quot;${DST}&quot;/>
				&lt;/air:SearchDestination>
				&lt;air:SearchDepTime PreferredTime=&quot;2022-01-23&quot;/>
			&lt;/air:SearchAirLeg>

			&lt;air:AirSearchModifiers>
				&lt;air:PreferredProviders>
					&lt;com:Provider Code=&quot;${PC}&quot;/>
				&lt;/air:PreferredProviders>
 &lt;air:PermittedCarriers>
               &lt;com:Carrier Code=&quot;${GlobalVariable.Carrier}&quot;/>
            &lt;/air:PermittedCarriers>

			&lt;/air:AirSearchModifiers>
			&lt;com:SearchPassenger Code=&quot;ADT&quot;/>
			&lt;!--&lt;com:SearchPassenger Code=&quot;ADT&quot;/>-->
          &lt;air:AirPricingModifiers InventoryRequestType=&quot;DirectAccess&quot;/>

			
		&lt;/air:LowFareSearchReq>
	&lt;/soap:Body>
&lt;/soap:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${GlobalVariable.EndPoint}/AirService</soapServiceEndpoint>
   <soapServiceFunction>service</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.TargetBranch</defaultValue>
      <description></description>
      <id>1b1248bd-b337-4fbc-89b9-d1ff5772e9ca</id>
      <masked>false</masked>
      <name>TB</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Org</defaultValue>
      <description></description>
      <id>25274eeb-4975-40c4-aab1-f21515aaf6aa</id>
      <masked>false</masked>
      <name>ORG</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Dst</defaultValue>
      <description></description>
      <id>c3b75a0b-ee13-45cd-b961-cf58301f7a60</id>
      <masked>false</masked>
      <name>DST</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ProviderCode</defaultValue>
      <description></description>
      <id>cad1dca4-5ad8-45e7-9a39-8d2e1aaad400</id>
      <masked>false</masked>
      <name>PC</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Password</defaultValue>
      <description></description>
      <id>4b0223d4-38b4-4c01-99ea-7f3d2568ec35</id>
      <masked>false</masked>
      <name>Password</name>
   </variables>
   <variables>
      <defaultValue>'UD_AGT_PP'</defaultValue>
      <description></description>
      <id>97272dbb-8cba-45bb-9cce-a45051adca7a</id>
      <masked>false</masked>
      <name>Username</name>
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

assertThat(response.getResponseText()).contains('Carrier')
assertThat(response.getResponseText()).contains('Origin')

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
